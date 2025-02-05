use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7119344: FileFormat = FileFormat {
    id: 7_119_344,
    source_type: SourceType::Wikidata,
    name: "PICtor PIC image format",
    extensions: &["clp", "pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
