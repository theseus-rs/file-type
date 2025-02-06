use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58959780: FileFormat = FileFormat {
    id: 58_959_780,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Chart, version 2",
    extensions: &["xlc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
