use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51718015: FileFormat = FileFormat {
    id: 51_718_015,
    puid: "wikidata/51718015",
    name: "Microsoft Excel OLE DB Query",
    extensions: &["rqy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
