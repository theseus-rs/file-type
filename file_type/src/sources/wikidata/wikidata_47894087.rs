use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47894087: FileFormat = FileFormat {
    id: 47_894_087,
    puid: "wikidata/47894087",
    name: "Microsoft Excel ODBC Query",
    extensions: &["dqy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
