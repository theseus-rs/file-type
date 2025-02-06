use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50223749: FileFormat = FileFormat {
    id: 50_223_749,
    source_type: SourceType::Wikidata,
    name: "compressed MusicXML",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    signatures: &[],
    related_formats: &[],
};
