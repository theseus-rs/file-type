use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2804859: FileFormat = FileFormat {
    id: 2_804_859,
    puid: "wikidata/2804859",
    name: "VDA-FS",
    extensions: &["vda", "vdafs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
