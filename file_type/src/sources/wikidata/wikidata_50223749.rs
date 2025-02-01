use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50223749: FileFormat = FileFormat {
    id: 50_223_749,
    puid: "wikidata/50223749",
    name: "compressed MusicXML",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    internal_signatures: &[],
    related_formats: &[],
};
