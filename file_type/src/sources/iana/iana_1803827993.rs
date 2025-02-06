use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1803827993: FileFormat = FileFormat {
    id: 1_803_827_993,
    source_type: SourceType::Iana,
    name: "gnap-binding-rotation-jws",
    extensions: &[],
    media_types: &["application/gnap-binding-rotation-jws"],
    signatures: &[],
    related_formats: &[],
};
