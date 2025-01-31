use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857250: FileFormat = FileFormat {
    id: 105_857_250,
    puid: "wikidata/105857250",
    name: "RoboHelp data",
    extensions: &["hpr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
