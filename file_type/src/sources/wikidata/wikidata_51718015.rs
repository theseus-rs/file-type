use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51718015: FileFormat = FileFormat {
    id: 51_718_015,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel OLE DB Query",
    extensions: &["rqy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
