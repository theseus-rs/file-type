use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28755730: FileFormat = FileFormat {
    id: 28_755_730,
    source_type: SourceType::Wikidata,
    name: "FDB (Legacy Family Tree)",
    extensions: &["fdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
