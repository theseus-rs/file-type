use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363690: FileFormat = FileFormat {
    id: 111_363_690,
    puid: "wikidata/111363690",
    name: "Yamaha Motif XS 'waves' format",
    extensions: &["x0w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
