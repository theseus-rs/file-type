use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904547: FileFormat = FileFormat {
    id: 29_904_547,
    puid: "wikidata/29904547",
    name: "Statistical Analysis System catalog",
    extensions: &["sas7bcat", "sc2", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
