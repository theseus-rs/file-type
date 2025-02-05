use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125938431: FileFormat = FileFormat {
    id: 125_938_431,
    source_type: SourceType::Wikidata,
    name: "Enigma Binary File 2",
    extensions: &["mus"],
    media_types: &["application/vnd.makemusic.notation"],
    signatures: &[],
    related_formats: &[],
};
