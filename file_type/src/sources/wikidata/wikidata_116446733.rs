use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116446733: FileFormat = FileFormat {
    id: 116_446_733,
    puid: "wikidata/116446733",
    name: "Microsoft Profit 1.0 Company File",
    extensions: &["pft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
