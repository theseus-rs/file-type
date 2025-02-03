use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_857512: FileFormat = FileFormat {
    id: 857_512,
    source_type: SourceType::Wikidata,
    name: "Smacker video",
    extensions: &["smk"],
    media_types: &["video/vnd.radgamettools.smacker"],
    internal_signatures: &[],
    related_formats: &[],
};
