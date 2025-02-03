use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126166135: FileFormat = FileFormat {
    id: 126_166_135,
    source_type: SourceType::Wikidata,
    name: "Compressed MusicXML 3.1+",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    internal_signatures: &[],
    related_formats: &[],
};
