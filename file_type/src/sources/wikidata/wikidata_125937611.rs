use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125937611: FileFormat = FileFormat {
    id: 125_937_611,
    source_type: SourceType::Wikidata,
    name: "Enigma Binary File 1",
    extensions: &["mus"],
    media_types: &["application/vnd.makemusic.notation"],
    signatures: &[],
    related_formats: &[],
};
