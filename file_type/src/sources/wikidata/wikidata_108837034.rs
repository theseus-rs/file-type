use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837034: FileFormat = FileFormat {
    id: 108_837_034,
    puid: "wikidata/108837034",
    name: "Nero Video-CD Compilation",
    extensions: &["nrv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
