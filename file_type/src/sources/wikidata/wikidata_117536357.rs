use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117536357: FileFormat = FileFormat {
    id: 117_536_357,
    puid: "wikidata/117536357",
    name: "ArcSoft Album and SlideShow Files",
    extensions: &["abm", "sld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
