use wasmdump::wasm_components::sections::{
    CustomSectionPayload, SectionCommonInterface, TypeEntry,
};
use wasmdump::wasm_components::types::ExternalKind;
use wasmdump::wasm_components::{base::Sizeof, module::WasmModule};

pub fn print_type_section(wasm_module: &WasmModule) {
    let type_section = wasm_module.get_type_section();

    if type_section.is_none() {
        println!("[Type Section (0 bytes)] None");
        return;
    }

    let type_section = type_section.unwrap();
    println!("[Type Section ({} bytes)]", type_section.sizeof());

    for (cnt, func_type) in type_section.get_type_list().into_iter().enumerate() {
        println!("  {}: {}", cnt, func_type);
    }
}

pub fn print_import_section(wasm_module: &WasmModule) {
    let import_section = wasm_module.get_import_section();
    let type_section = wasm_module.get_type_section().unwrap();

    if import_section.is_none() {
        println!("[Import Section (0 bytes)] None");
        return;
    }

    let import_section = import_section.unwrap();
    println!("[Import Section ({} bytes)]", import_section.sizeof());

    let import_entries = import_section.get_import_entries();
    for (cnt, import_entry) in import_entries.into_iter().enumerate() {
        match import_entry.get_type() {
            TypeEntry::FuncIndex { type_ } => {
                let type_idx = *type_;
                let func_type = type_section.get_type(type_idx as usize).unwrap();
                println!("  {}: (Function) {}", cnt, func_type);
            }
            // 処理をまとめたい
            TypeEntry::TblType { type_ } => {
                println!("  {}: (Table) {}", cnt, type_)
            }
            TypeEntry::MemType { type_ } => {
                println!("  {}: (Memory) {}", cnt, type_)
            }
            TypeEntry::GblType { type_ } => {
                println!("  {}: (Global) {}", cnt, type_)
            }
        };
    }
}

pub fn print_function_section(wasm_module: &WasmModule) {
    let func_section = wasm_module.get_function_section();
    let type_section = wasm_module.get_type_section().unwrap();
    let import_section = wasm_module.get_import_section();

    if func_section.is_none() {
        println!("[Function Section (0 bytes)] None");
        return;
    }

    let func_section = func_section.unwrap();
    println!("[Function Section ({} bytes)]", func_section.sizeof());

    let base_func_idx = match import_section {
        Some(sec) => sec.get_num_import_entries(),
        None => 0 as u32,
    };
    let type_indices = func_section.get_indice_list();
    for (cnt, type_idx) in type_indices.iter().enumerate() {
        let func_type = type_section.get_type(*type_idx as usize).unwrap();
        println!(
            "  {}: {}, func_idx={}, func_idx(rel)={}",
            cnt,
            func_type,
            base_func_idx + cnt as u32,
            cnt
        );
    }
}

pub fn print_table_section(wasm_module: &WasmModule) {
    let table_section = wasm_module.get_table_section();

    if table_section.is_none() {
        println!("[Table Section (0 bytes)] None");
        return;
    }

    let table_section = table_section.unwrap();
    println!("[Table Section ({} bytes)]", table_section.sizeof());

    let table_type = table_section.get_table_type(0); // wasm v1ではテーブルは一つのみ

    match table_type {
        Some(ty) => println!("  table type: {}", ty),
        None => println!(""),
    };
}

pub fn print_memory_section(wasm_module: &WasmModule) {
    let memory_section = wasm_module.get_memory_section();

    if memory_section.is_none() {
        println!("[Memory Section (0 bytes)] None");
        return;
    }

    let memory_section = memory_section.unwrap();
    println!("[Memory Section ({} bytes)]", memory_section.sizeof());

    let memory_info = memory_section.get_memory(0); // wasm v1では、線形メモリは一つのみ

    match memory_info {
        Some(mem) => println!("  memory limits: {}", mem.get_limits()),
        None => println!(""),
    };
}

pub fn print_global_section(wasm_module: &WasmModule) {
    let global_section = wasm_module.get_global_section();

    if global_section.is_none() {
        println!("[Global Section (0 bytes)] None");
        return;
    }

    let global_section = global_section.unwrap();
    println!("[Global Section ({} bytes)]", global_section.sizeof());

    let global_variables = global_section.get_global_variable_list();
    for (cnt, global_var) in global_variables.into_iter().enumerate() {
        println!("  {}: {}", cnt, global_var.get_global_type());
    }
}

pub fn print_export_section(wasm_module: &WasmModule) {
    let export_section = wasm_module.get_export_section();
    let import_section = wasm_module.get_import_section();
    let func_section = wasm_module.get_function_section();
    let global_section = wasm_module.get_global_section();
    let table_section = wasm_module.get_table_section();
    let type_section = wasm_module.get_type_section();

    if export_section.is_none() {
        println!("[Export Section (0 bytes)] None");
        return;
    }

    let export_section = export_section.unwrap();
    println!("[Export Section ({} bytes)]", export_section.sizeof());

    let export_entries = export_section.get_export_entry_list();
    for export_entry in export_entries.into_iter() {
        let entry_name = export_entry.get_entry_name();
        match export_entry.get_kind() {
            ExternalKind::Function => {
                let base_func_idx = match import_section {
                    Some(sec) => sec.get_num_import_entries(),
                    None => 0 as u32,
                };

                let func_idx = export_entry.get_index();
                let func_idx_rel = func_idx - base_func_idx;
                let idx_of_func_type = func_section
                    .unwrap()
                    .get_indice(func_idx_rel as usize)
                    .unwrap();
                let func_type = type_section
                    .unwrap()
                    .get_type(idx_of_func_type as usize)
                    .unwrap();

                println!(
                    "  (Function) {}: {}, func_idx={}, func_idx(rel)={}",
                    entry_name, func_type, func_idx, func_idx_rel
                );
            }
            ExternalKind::Global => {
                let idx = export_entry.get_index() as usize;
                let global_var_type = match global_section.unwrap().get_global_variable_type(idx) {
                    Some(gvt) => gvt,
                    None => panic!(
                        "> Error: unexpected error: export entry (idx: {}) not found",
                        idx
                    ),
                };
                let mut_flg = global_var_type.get_mutability();
                let val_type = global_var_type.get_type();

                println!(
                    "  (Global) {}:{}{}",
                    entry_name,
                    if mut_flg { " mut" } else { " " },
                    val_type
                );
            }
            ExternalKind::Memory => {
                println!("  (Memory) {}", entry_name);
            }
            ExternalKind::Table => {
                let idx = export_entry.get_index() as usize;
                println!(
                    "  (Table) {}: {:?}",
                    entry_name,
                    table_section.unwrap().get_table_type(idx).unwrap()
                );
            }
        };
    }
}

pub fn print_start_section(wasm_module: &WasmModule) {
    let start_section = wasm_module.get_start_section();

    if start_section.is_none() {
        println!("[Start Section (0 bytes)] None");
        return;
    }

    let start_section = start_section.unwrap();
    println!("[Start Section ({} bytes)]", start_section.sizeof());

    println!("  start: {}", start_section.get_start_func_index());
}

pub fn print_element_section(wasm_module: &WasmModule) {
    let elem_section = wasm_module.get_element_section();
    let type_section = wasm_module.get_type_section().unwrap();
    let func_section = wasm_module.get_function_section().unwrap();

    if elem_section.is_none() {
        println!("[Element Section (0 bytes)] None");
        return;
    }

    let elem_section = elem_section.unwrap();
    println!("[Element Section ({} bytes)]", elem_section.sizeof());

    for elem_idx in 0..elem_section.get_num_elements() {
        let elem_entries = elem_section
            .get_element(elem_idx as usize)
            .unwrap()
            .get_elements();
        let base_func_idx = match wasm_module.get_import_section() {
            Some(sec) => sec.get_num_import_entries(),
            None => 0 as u32,
        };
        for (cnt, func_idx) in elem_entries.into_iter().enumerate() {
            let func_idx_rel = func_section
                .get_indice(func_idx as usize - base_func_idx as usize)
                .unwrap();
            let func_type = type_section.get_type(func_idx_rel as usize).unwrap();
            println!(
                "  {}-{}: {}, func_idx(rel): {}",
                elem_idx, cnt, func_type, func_idx_rel
            );
        }
    }
}

pub fn print_data_section(wasm_module: &WasmModule) {
    let data_section = wasm_module.get_data_section();

    if data_section.is_none() {
        println!("[Data Section (0 bytes)] None");
        return;
    }

    let data_section = data_section.unwrap();
    println!("[Data Section ({} bytes)]", data_section.sizeof());

    let data_segments = data_section.get_data_segment_list();
    for (cnt, data_entry) in data_segments.into_iter().enumerate() {
        println!(
            "  {}: mem_idx={}, data_size={}",
            cnt,
            data_entry.get_memory_index(),
            data_entry.get_data_size()
        );
    }
}

pub fn print_custom_sections(wasm_module: &WasmModule) {
    let custom_sections = wasm_module.get_custom_sections();

    for custom_section in custom_sections.into_iter() {
        println!(
            "[Custom Section ({} bytes)] {}",
            custom_section.sizeof(),
            custom_section.get_name().unwrap()
        );

        match custom_section.get_payload() {
            CustomSectionPayload::Name { payload } => {
                if let Some(module_name) = payload.get_module_name() {
                    println!("  {}: (Module) {}", 0, module_name.get_name());
                }

                if let Some(function_names) = payload.get_function_names() {
                    let func_map = function_names.get_function_map();
                    for (cnt, naming) in func_map.get_name_list().into_iter().enumerate() {
                        println!(
                            "  {}: (function) {}, index={}",
                            cnt,
                            naming.get_name_str(),
                            naming.get_indice()
                        );
                    }
                }

                if let Some(local_names) = payload.get_local_names() {
                    let locals = local_names.get_locals();
                    for local_name in locals.into_iter() {
                        let func_idx = local_name.get_indice();
                        let local_map = local_name.get_local_map();
                        for (cnt, naming) in local_map.get_name_list().into_iter().enumerate() {
                            println!(
                                "  {}: (local) {}, func_idx={}, index={}",
                                cnt,
                                naming.get_name_str(),
                                func_idx,
                                naming.get_indice()
                            );
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn print_all_section_for_debug(wasm_module: &WasmModule) {
    let magic_and_version = wasm_module.get_magic_and_version();
    println!(
        "Magic and Version: ({} bytes)\n{:?}",
        magic_and_version.sizeof(),
        magic_and_version
    );

    if let Some(type_section) = wasm_module.get_type_section() {
        println!(
            "TypeSection: ({} bytes)\n{:?}\n",
            type_section.sizeof(),
            type_section
        );
    };

    if let Some(import_section) = wasm_module.get_import_section() {
        println!(
            "ImportSection: ({} bytes)\n{:?}\n",
            import_section.sizeof(),
            import_section
        );
    };

    if let Some(function_section) = wasm_module.get_function_section() {
        println!(
            "FunctionSection: ({} bytes)\n{:?}\n",
            function_section.sizeof(),
            function_section
        );
    };

    if let Some(table_section) = wasm_module.get_table_section() {
        println!(
            "TableSection: ({} bytes)\n{:?}\n",
            table_section.sizeof(),
            table_section
        );
    };

    if let Some(memory_section) = wasm_module.get_memory_section() {
        println!(
            "MemorySection: ({} bytes)\n{:?}\n",
            memory_section.sizeof(),
            memory_section
        );
    };

    if let Some(global_section) = wasm_module.get_global_section() {
        println!(
            "GlobalSection: ({} bytes)\n{:?}\n",
            global_section.sizeof(),
            global_section
        );
    };

    if let Some(export_section) = wasm_module.get_export_section() {
        println!(
            "ExportSection: ({} bytes)\n{:?}\n",
            export_section.sizeof(),
            export_section
        );
    };

    if let Some(start_section) = wasm_module.get_start_section() {
        println!(
            "StartSection: ({} bytes)\n{:?}\n",
            start_section.sizeof(),
            start_section
        );
    };

    if let Some(element_section) = wasm_module.get_element_section() {
        println!(
            "ElementSection: ({} bytes)\n{:?}\n",
            element_section.sizeof(),
            element_section
        );
    };

    if let Some(code_section) = wasm_module.get_code_section() {
        // println!("CodeSection:\n{:?}\n", code_section);
        println!(
            "CodeSection: ({} bytes)\nCodeSection {{ id: {}, payload_len: {} }}\nSkip (too large)\n",
            code_section.sizeof(),
            code_section.get_id(),
            code_section.get_payload_len()
        );
    };

    if let Some(data_section) = wasm_module.get_data_section() {
        // println!("DataSection:\n{:?}\n", data_section);
        println!(
            "DataSection: ({} bytes)\nDataSection {{ id: {}, payload_len: {} }}\nSkip (too large)\n",
            data_section.sizeof(),
            data_section.get_id(),
            data_section.get_payload_len()
        );
    };

    for custom_section in wasm_module.get_custom_sections().into_iter() {
        // println!("CustomSection:\n{:?}\n", custom_section);
        let name = custom_section.get_name().unwrap();

        if name == "name" {
            println!(
                "CustomSection: ({} bytes) '{:?}', common_size: {}\nSkip (too large and not parsed)\n",
                custom_section.sizeof(),
                name,
                custom_section.get_base().sizeof()
            );
        } else {
            println!(
                "CustomSection: ({} bytes) '{:?}'\nSkip (too large and not parsed)\n",
                custom_section.sizeof(),
                name
            );
        }
    }

    println!("\nSizeof WasmModule: {} bytes", wasm_module.sizeof());
}
