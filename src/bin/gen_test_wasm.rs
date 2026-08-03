/// Minimal valid WebAssembly module generator for testing.
///
/// Uses a hand-crafted minimal WASM binary with one exported function `add_one`
/// that takes an i32 and returns an i32.
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = generate_minimal_wasm();
    let out_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "tests/fixtures/minimal.wasm".to_string());
    let mut f = std::fs::File::create(&out_path)?;
    f.write_all(&wasm_bytes)?;
    eprintln!(
        "Generated test WASM: {} ({} bytes)",
        out_path,
        wasm_bytes.len()
    );
    Ok(())
}

/// Generate a minimal valid WASM module with an exported `add_one` function.
///
/// The module has:
/// - 1 type: (i32) -> i32
/// - 1 function using type 0
/// - 1 export: "add_one" → function 0
/// - Function body: local.get 0, i32.const 1, i32.add, end
fn generate_minimal_wasm() -> Vec<u8> {
    let mut wasm = Vec::new();

    // ── WASM header ─────────────────────────────────────────────────
    // Magic number \0asm + version 1
    wasm.extend_from_slice(b"\0asm");
    wasm.extend_from_slice(&1u32.to_le_bytes());

    // ── Type section (id 1) ─────────────────────────────────────────
    // 1 type: (i32) -> i32
    // Content: count(leb128_u32) + functype + param_count + param + result_count + result
    //        = 0x01 + 0x60 + 0x01 + 0x7f + 0x01 + 0x7f
    //        = 6 bytes
    wasm.push(0x01); // section id: Type
    write_leb_u32(&mut wasm, 6); // section size
    wasm.push(0x01);
    wasm.push(0x60); // 1 type, functype
    wasm.push(0x01);
    wasm.push(0x7f); // 1 param, i32
    wasm.push(0x01);
    wasm.push(0x7f); // 1 result, i32

    // ── Function section (id 3) ─────────────────────────────────────
    // 1 function, type index 0
    // Content: count + index = 0x01 + 0x00 = 2 bytes
    wasm.push(0x03); // section id: Function
    write_leb_u32(&mut wasm, 2); // section size
    wasm.push(0x01);
    wasm.push(0x00); // 1 function, type 0

    // ── Export section (id 7) ───────────────────────────────────────
    // 1 export: name="add_one" (7 chars), Func kind, index 0
    let export_name = b"add_one";
    // Content (section body): count(LEB) + name_len(LEB) + name + kind + idx(LEB)
    //   = 1 + 1 + 7 + 1 + 1 = 11 bytes
    let mut export_content = Vec::new();
    write_leb_u32(&mut export_content, 1); // 1 export (LEB128)
    write_leb_u32(&mut export_content, export_name.len() as u32); // name length (LEB128)
    export_content.extend_from_slice(export_name); // name bytes
    export_content.push(0x00); // export kind: Func
    write_leb_u32(&mut export_content, 0); // function index 0 (LEB128)
    wasm.push(0x07); // section id: Export
    write_leb_u32(&mut wasm, export_content.len() as u32); // section size
    wasm.extend_from_slice(&export_content); // section content

    // ── Code section (id 10) ────────────────────────────────────────
    // 1 function body.
    // Body: 0 locals + local.get 0 + i32.const 1 + i32.add + end
    let body_bytes: Vec<u8> = vec![
        0x00, // 0 local declarations
        0x20, 0x00, // local.get 0
        0x41, 0x01, // i32.const 1
        0x6a, // i32.add
        0x0b, // end
    ];

    // Each function body in the code section is: body_size(leb128_u32) + body
    // The body_size includes the local declarations + bytecode

    wasm.push(0x0a); // section id: Code
    // Code section content: count + (body_size + body) for each function
    // body_size = body_bytes.len() = 7
    // func_entry = body_size + body = 1 + 7 = 8 bytes (but body_size itself is 1-byte LEB for 7)
    // code_content = count(1) + body_size(1) + body(7) = 9 bytes
    let code_content_size = 1 + 1 + body_bytes.len() as u32; // count + body_size_leb + body
    write_leb_u32(&mut wasm, code_content_size); // section size
    wasm.push(0x01); // 1 function body
    write_leb_u32(&mut wasm, body_bytes.len() as u32); // body size
    wasm.extend_from_slice(&body_bytes); // body content

    wasm
}

/// Encode a u32 as LEB128 unsigned and append to buffer.
fn write_leb_u32(buf: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
}
