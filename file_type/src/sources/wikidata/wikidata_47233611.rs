use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47233611: FileFormat = FileFormat {
    id: 47_233_611,
    source_type: SourceType::Wikidata,
    name: "MPD",
    extensions: &["mpd"],
    media_types: &["application/x-multi-part-ldraw"],
    signatures: &[],
    related_formats: &[],
};
