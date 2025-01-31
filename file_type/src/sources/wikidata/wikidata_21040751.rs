use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21040751: FileFormat = FileFormat {
    id: 21_040_751,
    puid: "wikidata/21040751",
    name: "Farandole Composer format",
    extensions: &["far"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
