use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126166135: FileFormat = FileFormat {
    id: 126_166_135,
    source_type: SourceType::Wikidata,
    name: "Compressed MusicXML 3.1+",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    signatures: &[],
    related_formats: &[],
};
