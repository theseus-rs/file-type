use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857349: FileFormat = FileFormat {
    id: 105_857_349,
    puid: "wikidata/105857349",
    name: "Qt Jambi User Interface",
    extensions: &["jui"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
