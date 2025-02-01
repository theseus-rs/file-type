use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_77433095: FileFormat = FileFormat {
    id: 77_433_095,
    puid: "wikidata/77433095",
    name: "YaST MetaPackage",
    extensions: &["ymp"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
