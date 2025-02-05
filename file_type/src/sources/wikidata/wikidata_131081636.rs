use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131081636: FileFormat = FileFormat {
    id: 131_081_636,
    source_type: SourceType::Wikidata,
    name: "Snowball source code file",
    extensions: &["sbl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
