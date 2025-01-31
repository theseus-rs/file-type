use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272291: FileFormat = FileFormat {
    id: 111_272_291,
    puid: "wikidata/111272291",
    name: "Ensoniq SQ1/SQ2/KS32 disk image",
    extensions: &["edq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
