use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110620022: FileFormat = FileFormat {
    id: 110_620_022,
    puid: "wikidata/110620022",
    name: "Adobe Atmosphere world (.aer)",
    extensions: &["aer"],
    media_types: &["http://www.wikidata.org/.well-known/genid/64e48b81d07f7c6707af81a93ee3a882"],
    internal_signatures: &[],
    related_formats: &[],
};
