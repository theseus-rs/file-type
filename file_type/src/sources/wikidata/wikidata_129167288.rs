use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129167288: FileFormat = FileFormat {
    id: 129_167_288,
    puid: "wikidata/129167288",
    name: "execline file format",
    extensions: &["exec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
