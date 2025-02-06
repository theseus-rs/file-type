use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129167999: FileFormat = FileFormat {
    id: 129_167_999,
    source_type: SourceType::Wikidata,
    name: "Factor source code file",
    extensions: &["factor"],
    media_types: &["text/x-factor"],
    signatures: &[],
    related_formats: &[],
};
