use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129825037: FileFormat = FileFormat {
    id: 129_825_037,
    source_type: SourceType::Wikidata,
    name: "Io source code file",
    extensions: &["io"],
    media_types: &["text/x-iosrc"],
    signatures: &[],
    related_formats: &[],
};
