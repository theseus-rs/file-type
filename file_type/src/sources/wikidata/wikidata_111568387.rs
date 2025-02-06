use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111568387: FileFormat = FileFormat {
    id: 111_568_387,
    source_type: SourceType::Wikidata,
    name: "Managed Object Format",
    extensions: &["mof"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
