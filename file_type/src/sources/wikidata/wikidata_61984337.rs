use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61984337: FileFormat = FileFormat {
    id: 61_984_337,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro database container (table files)",
    extensions: &["dbc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
