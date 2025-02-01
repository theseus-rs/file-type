use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130543516: FileFormat = FileFormat {
    id: 130_543_516,
    puid: "wikidata/130543516",
    name: "PyPy log file format",
    extensions: &["pypylog"],
    media_types: &["application/x-pypylog"],
    internal_signatures: &[],
    related_formats: &[],
};
