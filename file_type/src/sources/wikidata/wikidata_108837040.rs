use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837040: FileFormat = FileFormat {
    id: 108_837_040,
    puid: "wikidata/108837040",
    name: "Nero DVD-Video Compilation File",
    extensions: &["nrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
