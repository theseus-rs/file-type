use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99972444: FileFormat = FileFormat {
    id: 99_972_444,
    puid: "wikidata/99972444",
    name: "Advanced Disk Catalog",
    extensions: &["adc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
