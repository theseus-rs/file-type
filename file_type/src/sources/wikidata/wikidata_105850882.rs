use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850882: FileFormat = FileFormat {
    id: 105_850_882,
    source_type: SourceType::Wikidata,
    name: "Kazaa Playlist",
    extensions: &["kpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
