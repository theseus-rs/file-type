use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104600902: FileFormat = FileFormat {
    id: 104_600_902,
    source_type: SourceType::Wikidata,
    name: "KDB",
    extensions: &["kdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
