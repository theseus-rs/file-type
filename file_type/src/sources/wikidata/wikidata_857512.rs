use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_857512: FileFormat = FileFormat {
    id: 857_512,
    source_type: SourceType::Wikidata,
    name: "Smacker video",
    extensions: &["smk"],
    media_types: &["video/vnd.radgamettools.smacker"],
    signatures: &[],
    related_formats: &[],
};
