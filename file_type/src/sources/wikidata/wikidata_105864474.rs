use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864474: FileFormat = FileFormat {
    id: 105_864_474,
    puid: "wikidata/105864474",
    name: "PSpice Probe configuration",
    extensions: &["prb"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
