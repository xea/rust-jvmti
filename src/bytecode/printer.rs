use super::classfile::*;

pub struct ClassfilePrinter;

impl ClassfilePrinter {
    pub fn render_lines(classfile: &Classfile) -> Vec<String> {
        let mut lines = vec![];

        lines.push(format!("class {}", ClassfilePrinter::resolve_class(&classfile.this_class, &classfile.constant_pool)));
        lines.push(format!("Minor version: {}", classfile.version.minor_version));
        lines.push(format!("Major version: {}", classfile.version.major_version));
        lines.push(format!("Flags: {}", ClassfilePrinter::render_flags(&classfile.access_flags)));
        lines.push(format!("Constant pool:"));

        let mut i: i32 = -1;

        let _: Vec<()> = ClassfilePrinter::render_constant_pool(&classfile.constant_pool).iter()
            .map(|constant| {
                i = i + 1;
                format!("{:>5} = {}", format!("#{}", i), constant)
            })
            .map(|line| lines.push(line))
            .collect();

        lines
    }

    pub fn render_flags(flags: &AccessFlags) -> String {
        let mut flag_vec = vec![];

        if flags.has_flag(ClassAccessFlags::Public as u16) {
            flag_vec.push("ACC_PUBLIC ");
        }

        if flags.has_flag(ClassAccessFlags::Super as u16) {
            flag_vec.push("ACC_SUPER ");
        }

        // TODO implement other access flags

        flag_vec.iter().fold(String::new(), |mut acc, x| { acc.push_str(x); acc })
    }

    pub fn render_constant_pool(constant_pool: &ConstantPool) -> Vec<String> {
        constant_pool.constants.iter().map(|constant| {
            ClassfilePrinter::render_constant(&constant, constant_pool)
        }).collect()
    }

    pub fn render_constant(constant: &Constant, pool: &ConstantPool) -> String {
        match constant {
            &Constant::Utf8(ref content) => format!("Utf8               {}", String::from_utf8_lossy(content.as_slice())),
            &Constant::Integer(value) => format!("Integer            {}", value),
            &Constant::Float(value) => format!("Float               {}", value),
            &Constant::Long(value) => format!("Long               {}", value),
            &Constant::Double(value) => format!("Double              {}", value),
            &Constant::Class(ref index) => format!("Class              #{:<24}// {}", index.idx, ClassfilePrinter::resolve_utf8(index, pool)),
            &Constant::FieldRef { class_index: ref ci, name_and_type_index: ref ni } => format!("FieldRef           {:<24} // {}.{}", format!("#{}.#{}", ci.idx, ni.idx), ClassfilePrinter::resolve_class(ci, pool), ClassfilePrinter::resolve_name_and_type(ni, &pool)),
            &Constant::MethodRef { class_index: ref ci, name_and_type_index: ref ni } => format!("MethodRef          {:<24} // {}.{}", format!("#{}.#{}", ci.idx, ni.idx), ClassfilePrinter::resolve_class(ci, pool), ClassfilePrinter::resolve_name_and_type(ni, pool)),
            &Constant::InterfaceMethodRef { class_index: ref ci, name_and_type_index: ref ni } => format!("InterfaceMethodRef {:<24} // {}.{}", format!("#{}.#{}", ci.idx, ni.idx), ClassfilePrinter::resolve_class(ci, pool), ClassfilePrinter::resolve_name_and_type(ni, pool)),
            &Constant::String(ref cpi) => format!("String             #{:<24}// {}", cpi.idx, ClassfilePrinter::resolve_utf8(cpi, pool)),
            &Constant::NameAndType { name_index: ref ni, descriptor_index: ref dp } => format!("NameAndType        {:<24} // {}:{}", format!("#{}.#{}", ni.idx, dp.idx), ClassfilePrinter::resolve_utf8(ni, pool), ClassfilePrinter::resolve_utf8(dp, pool)),
            &Constant::MethodHandle { reference_kind: ref kind, reference_index: ref ri } => format!("MethodHandle       {} #{}", ClassfilePrinter::resolve_reference_kind(kind), ri.idx),
            &Constant::MethodType(ref cpi) => format!("MethodType         #{}", cpi.idx),
            &Constant::InvokeDynamic { bootstrap_method_attr_index: ref bi, name_and_type_index: ref ni } => format!("InvokeDynamic      #{}.{}", bi.idx, ClassfilePrinter::resolve_name_and_type(ni, pool)),
            &Constant::Unknown(value) => format!("Unknown constant        {}", value),
            &Constant::Placeholder => format!("Placeholder")
        }
    }

    pub fn render_utf8(index: &ConstantPoolIndex, cp: &ConstantPool) -> Option<String> {
        cp.get_utf8_string(index.idx as u16)
    }

    pub fn resolve_utf8(index: &ConstantPoolIndex, cp: &ConstantPool) -> String {
        ClassfilePrinter::render_utf8(index, cp).unwrap_or(String::from("<Not UTF8>"))
    }

    pub fn resolve_class(index: &ConstantPoolIndex, cp: &ConstantPool) -> String {
        cp.resolve_index(index).map(|constant| match constant {
            &Constant::Class(ref idx) => ClassfilePrinter::resolve_utf8(idx, cp),
            _ => String::from("<Not a class>")
        }).unwrap_or(String::from("<Not found>"))
    }

    pub fn resolve_name_and_type(nandt: &ConstantPoolIndex, cp: &ConstantPool) -> String {
        cp.resolve_index(nandt).map(|constant| match constant {
            &Constant::NameAndType { name_index: ref ni, descriptor_index: ref di } => format!("{}:{}", ClassfilePrinter::resolve_utf8(ni, cp), ClassfilePrinter::resolve_utf8(di, cp)),
            _ => String::from("<Not a name and type index>")
        }).unwrap_or(String::from("<Not found>"))
    }

    pub fn resolve_reference_kind(kind: &ReferenceKind) -> String {
        String::from(match kind {
            &ReferenceKind::GetField => "GetField",
            &ReferenceKind::GetStatic => "GetStatic",
            &ReferenceKind::InvokeInterface => "InvokeInterface",
            &ReferenceKind::InvokeSpecial => "InvokeSpecial",
            &ReferenceKind::InvokeStatic => "InvokeStatic",
            &ReferenceKind::InvokeVirtual => "InvokeVirtual",
            &ReferenceKind::NewInvokeSpecial => "NewInvokeSpecial",
            &ReferenceKind::PutField => "PutField",
            &ReferenceKind::PutStatic => "PutStatic",
            _ => "Unknown"
        })
    }
}
