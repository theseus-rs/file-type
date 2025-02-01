use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51093476: FileFormat = FileFormat {
    id: 51_093_476,
    puid: "wikidata/51093476",
    name: "Microsoft Excel OLAP Query",
    extensions: &["oqy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
