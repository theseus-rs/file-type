use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47894087: FileFormat = FileFormat {
    id: 47_894_087,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel ODBC Query",
    extensions: &["dqy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
