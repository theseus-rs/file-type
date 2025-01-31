use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118489528: FileFormat = FileFormat {
    id: 118_489_528,
    puid: "wikidata/118489528",
    name: "Microsoft Excel Workspace File 5/95",
    extensions: &["xlw"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
