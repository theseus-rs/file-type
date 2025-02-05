use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111667275: FileFormat = FileFormat {
    id: 111_667_275,
    source_type: SourceType::Wikidata,
    name: "OEW objectbase",
    extensions: &["oew"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
