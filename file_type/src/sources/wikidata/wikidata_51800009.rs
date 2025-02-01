use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51800009: FileFormat = FileFormat {
    id: 51_800_009,
    puid: "wikidata/51800009",
    name: "Microsoft Excel Macro, version 4",
    extensions: &["xla", "xlm"],
    media_types: &["application/vnd.ms-excel", "application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
