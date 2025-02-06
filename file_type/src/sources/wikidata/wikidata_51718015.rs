use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51718015: FileFormat = FileFormat {
    id: 51_718_015,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel OLE DB Query",
    extensions: &["rqy"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
