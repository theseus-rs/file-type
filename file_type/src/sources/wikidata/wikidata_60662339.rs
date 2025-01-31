use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60662339: FileFormat = FileFormat {
    id: 60_662_339,
    puid: "wikidata/60662339",
    name: "AutoCAD Plot Configuration File, version 1",
    extensions: &["pcp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
