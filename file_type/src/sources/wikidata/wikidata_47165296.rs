use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47165296: FileFormat = FileFormat {
    id: 47_165_296,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project file format, version 98",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    signatures: &[],
    related_formats: &[],
};
