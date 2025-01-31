use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34743161: FileFormat = FileFormat {
    id: 34_743_161,
    puid: "wikidata/34743161",
    name: "Softdisk Publishing UDF",
    extensions: &["udf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
