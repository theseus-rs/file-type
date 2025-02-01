use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_97012602: FileFormat = FileFormat {
    id: 97_012_602,
    puid: "wikidata/97012602",
    name: "gnuplot plot files",
    extensions: &["gp", "gplt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
