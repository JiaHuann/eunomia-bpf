use super::CheckedExportedMember;

pub(crate) mod key_value;
pub(crate) mod simple_value;

pub(crate) fn get_plain_text_checked_types_header(
    checked_member: &mut [CheckedExportedMember],
    prev_header: impl AsRef<str>,
) -> String {
    let mut header = String::from(prev_header.as_ref());
    for ty in checked_member.iter_mut() {
        ty.output_header_offset = header.len();
        let type_name = ty
            .meta
            .name
            .chars()
            .into_iter()
            .map(|v| v.to_ascii_uppercase())
            .collect::<String>();
        header.push_str(&type_name);
        if type_name.len() < 6 {
            for _ in type_name.len()..6 {
                header.push(' ');
            }
        }
        header.push(' ');
    }
    header
}