use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837051: FileFormat = FileFormat {
    id: 108_837_051,
    puid: "wikidata/108837051",
    name: "Nero UDF CD-ROM Compilation",
    extensions: &["nru"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
