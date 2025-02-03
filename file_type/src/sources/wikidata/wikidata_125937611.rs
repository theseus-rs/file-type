use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125937611: FileFormat = FileFormat {
    id: 125_937_611,
    source_type: SourceType::Wikidata,
    name: "Enigma Binary File 1",
    extensions: &["mus"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
