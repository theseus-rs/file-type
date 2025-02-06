use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127266247: FileFormat = FileFormat {
    id: 127_266_247,
    source_type: SourceType::Wikidata,
    name: "Assembly file",
    extensions: &["eaf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
